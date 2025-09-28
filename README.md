# promptly
Offline text generator CLI using simple Markov chains.

- Core types and algorithms: [`markov::MarkovChain`](src/markov/chain.rs), [`markov::train_from_corpus`](src/markov/builder.rs)
- Model persistence (YAML): [`model::save_model`](src/model/mod.rs), [`model::load_model`](src/model/mod.rs)
- Corpus handling: [`corpus::load_text`](src/corpus/loader.rs), [`corpus::preprocess_text`](src/corpus/loader.rs)
- Entry point: [src/main.rs](src/main.rs)

## Why
- Zero-dependency generation at runtime (no network calls)
- Simple, inspectable models (YAML)
- Reproducible CLI workflows

## Install
Requires Rust and Cargo.

```sh
# build locally
cargo build --release

# or install into your Cargo bin
cargo install --path .
```

## Quick start
```sh
# Generate using the default corpus with order 2
promptly "hello world"

# Train from a specific corpus and save the model
promptly -c corpora/default.txt -o 2 --max-words 80 --save-model model.yaml "the quick"

# Load an existing model and generate
promptly --model model.yaml --max-words 120 "knowledge is"
```

## Usage
```sh
promptly [OPTIONS] <PROMPT>
```

Options:
- `-m, --max-words <N>`: Maximum number of words to generate (default: 100)
- `-o, --order <N>`: Markov chain order (n-gram size) (default: 2)
- `-c, --corpus <PATH>`: Training corpus file (default: corpora/default.txt)
- `--model <PATH>`: Load a pre-trained model instead of training
- `--save-model <PATH>`: Save the trained model to the given path
- `-v, --verbose`: No-op (use RUST_LOG for log level)

Notes:
- If `--model` is provided, the corpus is not used for training.
- If your prompt is shorter than the chain order, generation starts from a random known state.

Logging (recommended):
```sh
RUST_LOG=info promptly -c corpora/default.txt "start prompt"
RUST_LOG=debug promptly -o 3 "inspect behavior"
```

## Features
- Train an n-gram Markov chain from a text corpus (order 1–3 recommended).
- Generate text from a prompt or back off to a random state if needed.
- Save/load trained models as YAML.
- Optional auto-save of trained models via config when not loading an existing model.
- Logging via `env_logger` and `log`.

## Configuration
All configuration files live in the config/ directory.

- Default profile: config/default.yaml (automatically loaded at startup)
- Additional profiles: create more .yaml files in config/ (e.g., config/poetry.yaml, config/news.yaml)

Switching profiles:
- Replace the default profile with another file from the same directory:
  ```sh
  cp config/poetry.yaml config/default.yaml
  ```
- Or keep multiple files and swap as needed.

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

Behavior:
- Only config/default.yaml is auto-loaded.
- model.auto_save: if true and you did not pass --model, the app auto-saves to model_order_<order>.yaml.
- CLI flags take precedence over config values.

## Model format and persistence
Models are saved as YAML with:
- `order`: the chain order (n)
- `transitions`: map from state (Vec<String>) to next-word counts

APIs:
- Save: [`model::save_model`](src/model/mod.rs)
- Load: [`model::load_model`](src/model/mod.rs)

Auto-save path (when enabled): `model_order_<order>.yaml`.

## Examples
```sh
# Different orders
promptly -o 1 "markov chains are"
promptly -o 3 "markov chains are"

# Limit output aggressively
promptly -m 20 "short sample"

# Train from a custom corpus and use later
promptly -c /path/to/corpus.txt -o 2 --save-model my_model.yaml "seed"
promptly --model my_model.yaml "another prompt"
```

## Architecture overview
- Corpus I/O and preprocessing: [`corpus`](src/corpus/mod.rs)
- Markov training and generation: [`markov`](src/markov/mod.rs)
  - Chain type and generation: [`markov::MarkovChain`](src/markov/chain.rs)
  - Training helper: [`markov::train_from_corpus`](src/markov/builder.rs)
- Model I/O: [`model`](src/model/mod.rs)
- CLI args: [`cli::Args`](src/cli/args.rs)

Generation flow:
- Normalize prompt and derive initial state
- Weighted random next-word selection by observed counts
- Backoff to a random known state if the current state has no transitions

## Troubleshooting
- Empty or very short output:
  - Use a shorter order (`-o 1` or `-o 2`) or provide more training data.
- “Corpus file not found”:
  - Check the `-c` path and current working directory.
- “Model does not match prompt”:
  - Ensure the model was trained with an order compatible with your expectations; prompts shorter than `order` will start randomly.

## Development
- Rust edition: see [Cargo.toml](Cargo.toml)
- Logging: `RUST_LOG=debug|info|warn|error`
- Tests: none yet (scaffold via `dev-dependencies`)
- Run:
```sh
cargo run -- "a starting prompt"
cargo run --release -- "a starting prompt"
```

## License
MIT — see [LICENSE](LICENSE).
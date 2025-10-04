# promptly
Offline text generator CLI using simple Markov chains.

- Core types and algorithms: [`markov::MarkovChain`](src/markov/chain.rs), [`markov::train_from_corpus`](src/markov/builder.rs)
- Model persistence (JSON): [`model::save_model`](src/model/mod.rs), [`model::load_model`](src/model/mod.rs)
- Corpus handling: [`corpus::load_text`](src/corpus/loader.rs), [`corpus::preprocess_text`](src/corpus/loader.rs)
- Entry point: [src/main.rs](src/main.rs)

## Why
- No network calls at runtime
- Simple, inspectable models (JSON)
- Reproducible CLI workflows

## Install
Requires Rust and Cargo.

```sh
cargo build --release
# or install to Cargo bin
cargo install --path .
```

## Quick start
```sh
# Generate using the default corpus with order 2
promptly "hello world"

# Train from a specific corpus and save the model
promptly -c corpora/default.txt -o 2 --max-words 80 --save-model model.json "the quick"

# Load an existing model and generate
promptly --model models/model_order_2_1759552453.json --max-words 120 "knowledge is"
```

## Usage
```sh
promptly [OPTIONS] <PROMPT>
```

Options:
- `-m, --max-words <N>`: Maximum number of words to generate (default: 100)
- `-o, --order <N>`: Markov chain order (n-gram size) (default: 2)
- `-c, --corpus <PATH>`: Training corpus file (default: corpora/default.txt)
- `--corpus-files <PATH>...`: Multiple corpus files (parsed, not used in current training flow)
- `--corpus-dir <PATH>`: Directory of corpus files (parsed, not used in current training flow)
- `--model <PATH>`: Load a pre-trained model instead of training
- `--save-model <PATH>`: Save the trained model to the given path
- `-v, --verbose`: Parsed but no-op

// filepath: /home/soumil/Soumil/Programming/rust/promptly/README.md
# promptly
Offline text generator CLI using simple Markov chains.

- Core types and algorithms: [`markov::MarkovChain`](src/markov/chain.rs), [`markov::train_from_corpus`](src/markov/builder.rs)
- Model persistence (JSON): [`model::save_model`](src/model/mod.rs), [`model::load_model`](src/model/mod.rs)
- Corpus handling: [`corpus::load_text`](src/corpus/loader.rs), [`corpus::preprocess_text`](src/corpus/loader.rs)
- Entry point: [src/main.rs](src/main.rs)

## Why
- No network calls at runtime
- Simple, inspectable models (JSON)
- Reproducible CLI workflows

## Install
Requires Rust and Cargo.

```sh
cargo build --release
# or install to Cargo bin
cargo install --path .
```

## Quick start
```sh
# Generate using the default corpus with order 2
promptly "hello world"

# Train from a specific corpus and save the model
promptly -c corpora/default.txt -o 2 --max-words 80 --save-model model.json "the quick"

# Load an existing model and generate
promptly --model models/model_order_2_1759552453.json --max-words 120 "knowledge is"
```

## Usage
```sh
promptly [OPTIONS] <PROMPT>
```

Options:
- `-m, --max-words <N>`: Maximum number of words to generate (default: 100)
- `-o, --order <N>`: Markov chain order (n-gram size) (default: 2)
- `-c, --corpus <PATH>`: Training corpus file (default: corpora/default.txt)
- `--corpus-files <PATH>...`: Multiple corpus files (parsed, not used in current training flow)
- `--corpus-dir <PATH>`: Directory of corpus files (parsed, not used in current training flow)
- `--model <PATH>`: Load a pre-trained model instead of training
- `--save-model <PATH>`: Save the trained model to the given path
- `-v, --verbose`: Parsed but no-op

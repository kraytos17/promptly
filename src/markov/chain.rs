use crate::markov::interner::Interner;
use rand::{Rng, seq::IteratorRandom};
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Invalid state for generation")]
    InvalidState,
    #[error("No transitions available for current state")]
    NoTransitions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub next_words: Vec<usize>,
    pub counts: Vec<usize>,
    pub cumulative: Vec<usize>,
    #[serde(skip)]
    total: usize,
    #[serde(skip)]
    next_word_index: HashMap<usize, usize>,
}

impl State {
    pub fn new() -> Self {
        Self {
            next_words: Vec::new(),
            counts: Vec::new(),
            cumulative: Vec::new(),
            total: 0,
            next_word_index: HashMap::new(),
        }
    }

    pub fn update_cumulative(&mut self) {
        self.cumulative.clear();
        self.total = 0;
        for &cnt in &self.counts {
            self.total += cnt;
            self.cumulative.push(self.total);
        }
    }

    pub fn increment(&mut self, word: usize) {
        if let Some(&idx) = self.next_word_index.get(&word) {
            self.counts[idx] += 1;
        } else {
            let idx = self.next_words.len();
            self.next_words.push(word);
            self.counts.push(1);
            self.next_word_index.insert(word, idx);
        }
    }

    pub fn select_next(&self, rng: &mut impl Rng) -> Option<usize> {
        if self.cumulative.is_empty() || self.total == 0 {
            return None;
        }
        let choice = rng.random_range(0..self.total);
        match self.cumulative.binary_search(&choice) {
            Ok(idx) | Err(idx) => Some(self.next_words[idx]),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkovChain {
    pub order: usize,
    #[serde(with = "vec_key_map")]
    pub states: HashMap<Vec<usize>, State>,
    pub interner: Interner,
}

pub mod vec_key_map {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;

    pub fn serialize<S>(
        map: &HashMap<Vec<usize>, super::State>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_keyed: HashMap<String, &super::State> = map
            .iter()
            .map(|(k, v)| {
                (
                    k.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join("_"),
                    v,
                )
            })
            .collect();

        string_keyed.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<Vec<usize>, super::State>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_keyed = HashMap::<String, super::State>::deserialize(deserializer)?;
        Ok(string_keyed
            .into_iter()
            .map(|(k, v)| {
                (
                    k.split('_')
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<usize>>(),
                    v,
                )
            })
            .collect())
    }
}

impl MarkovChain {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            states: HashMap::new(),
            interner: Interner::new(),
        }
    }

    pub fn train(&mut self, text: &str) {
        let words: Vec<_> = text
            .split_whitespace()
            .map(|w| self.interner.get_or_intern(w))
            .collect();

        if words.len() <= self.order {
            return;
        }

        let mut state: VecDeque<usize> = words.iter().take(self.order).copied().collect();
        for &next_word in &words[self.order..] {
            let key = state.iter().copied().collect::<Vec<_>>();
            let node = self.states.entry(key).or_insert_with(State::new);
            node.increment(next_word);

            state.pop_front();
            state.push_back(next_word);
        }

        for node in self.states.values_mut() {
            node.update_cumulative();
        }
    }

    pub fn generate(&mut self, prompt: &str, max_words: usize) -> Result<String, GenerationError> {
        let mut rng = rand::rng();
        let mut state: VecDeque<usize> = prompt
            .split_whitespace()
            .map(|w| self.interner.get_or_intern(w))
            .collect();

        if state.len() < self.order {
            let random_state = self
                .states
                .keys()
                .choose(&mut rng)
                .ok_or(GenerationError::NoTransitions)?;
            state = VecDeque::from(random_state.clone());
        }

        while state.len() > self.order {
            state.pop_front();
        }

        let mut output: Vec<usize> = state.iter().copied().collect();
        for _ in 0..max_words {
            let Some(node) = self.states.get(state.as_slices().0) else {
                let random_state = self
                    .states
                    .keys()
                    .choose(&mut rng)
                    .ok_or(GenerationError::NoTransitions)?;

                state = VecDeque::from(random_state.clone());
                output.extend(state.iter());
                continue;
            };

            let Some(next_word) = node.select_next(&mut rng) else {
                break;
            };

            state.push_back(next_word);
            if state.len() > self.order {
                state.pop_front();
            }

            output.push(next_word);
        }

        let text = output
            .into_iter()
            .filter_map(|id| self.interner.resolve(id))
            .collect::<Vec<_>>()
            .join(" ");

        Ok(text)
    }

    pub fn _get_probability(&self, state: &[usize], next_word: usize) -> Option<f64> {
        let node = self.states.get(state)?;
        let total = node.total;
        let pos = node.next_word_index.get(&next_word)?;

        Some(node.counts[*pos] as f64 / total as f64)
    }
}

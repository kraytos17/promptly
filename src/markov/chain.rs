use rand::{Rng, seq::IteratorRandom};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Invalid state for generation")]
    InvalidState,
    #[error("No transitions available for current state")]
    NoTransitions,
}

#[derive(Debug, Clone)]
pub struct MarkovChain {
    pub order: usize,
    pub transitions: HashMap<Vec<String>, HashMap<String, usize>>,
}

impl MarkovChain {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            transitions: HashMap::new(),
        }
    }

    pub fn train(&mut self, text: &str) {
        let words: Vec<_> = text.split_whitespace().collect();
        if words.len() <= self.order {
            return;
        }

        for window in words.windows(self.order + 1) {
            let state: Vec<_> = window[..self.order]
                .iter()
                .map(ToString::to_string)
                .collect();
            let next_word = window[self.order].to_string();

            *self
                .transitions
                .entry(state)
                .or_default()
                .entry(next_word)
                .or_insert(0) += 1;
        }
    }

    pub fn generate(&self, prompt: &str, max_words: usize) -> Result<String, GenerationError> {
        let mut prompt_words: Vec<String> =
            prompt.split_whitespace().map(|w| w.to_string()).collect();

        if prompt_words.len() < self.order {
            let mut rng = rand::rng();
            let random_state = self
                .transitions
                .keys()
                .choose(&mut rng)
                .ok_or(GenerationError::NoTransitions)?;
            prompt_words = random_state.clone();
        }

        let mut curr_state = if prompt_words.len() >= self.order {
            prompt_words[prompt_words.len() - self.order..].to_vec()
        } else {
            prompt_words.clone()
        };

        let mut res = prompt_words.join(" ");
        for _ in 0..max_words {
            let next_word = match self.next_word(&curr_state) {
                Some(word) => word,
                None => {
                    let mut rng = rand::rng();
                    if let Some(random_state) = self.transitions.keys().choose(&mut rng) {
                        curr_state = random_state.clone();
                        continue;
                    } else {
                        break;
                    }
                }
            };

            res.push(' ');
            res.push_str(&next_word);

            if curr_state.len() >= self.order {
                curr_state.remove(0);
            }
            curr_state.push(next_word);
        }

        Ok(res)
    }

    pub fn next_word(&self, state: &[String]) -> Option<String> {
        let options = self.transitions.get(state)?;
        let tot_wt: usize = options.values().sum();
        if tot_wt == 0 {
            return None;
        }

        let mut rng = rand::rng();
        let mut random_val = rng.random_range(0..tot_wt);
        for (word, &wt) in options {
            if random_val < wt {
                return Some(word.clone());
            }
            random_val -= wt;
        }

        None
    }

    pub fn get_probability(&self, state: &[String], next_word: &str) -> Option<f64> {
        let options = self.transitions.get(state)?;
        let total = options.values().sum::<usize>() as f64;
        let count = *options.get(next_word)? as f64;

        Some(count / total)
    }
}

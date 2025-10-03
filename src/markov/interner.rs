use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Interner {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            word_to_id: HashMap::new(),
            id_to_word: Vec::new(),
        }
    }

    pub fn get_or_intern(&mut self, word: &str) -> usize {
        if let Some(&id) = self.word_to_id.get(word) {
            id
        } else {
            let id = self.id_to_word.len();
            self.word_to_id.insert(word.to_string(), id);
            self.id_to_word.push(word.to_string());

            id
        }
    }

    pub fn resolve(&self, id: usize) -> Option<&str> {
        self.id_to_word.get(id).map(String::as_str)
    }
}

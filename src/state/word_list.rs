use include_lines::include_lines;
use rand::prelude::*;
use std::collections::HashSet;

pub struct WordList {
    words: Vec<&'static str>,
    rng: rand::rngs::ThreadRng,
}

impl WordList {
    pub fn new(allowed_letters: &HashSet<char>) -> Self {
        const WORDS: [&'static str; 4974] = include_lines!("res/words.en.txt");

        let rng = rand::thread_rng();
        let matching = WORDS
            .into_iter()
            .filter(|w| Self::is_valid(w, allowed_letters))
            .collect::<Vec<_>>();

        Self {
            words: matching,
            rng,
        }
    }

    fn is_valid(word: &str, allowed_letters: &HashSet<char>) -> bool {
        for ch in word.chars() {
            if !allowed_letters.contains(&ch) {
                return false;
            }
        }
        true
    }

    pub fn next_word(&mut self) -> &'static str {
        self.words.choose(&mut self.rng).unwrap_or(&"hello")
    }
}

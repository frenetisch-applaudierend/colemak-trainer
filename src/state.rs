use std::{collections::HashSet, mem::replace};

use include_lines::include_lines;
use rand::prelude::*;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};

use crate::layout::{KeyboardLayout, LayoutMapper};

pub struct AppState {
    pub target_layout: KeyboardLayout,
    pub layout_mapper: LayoutMapper,
    word_list: WordList,
}

impl AppState {
    pub fn new(
        target_layout: KeyboardLayout,
        layout_mapper: LayoutMapper,
        word_list: WordList,
    ) -> Self {
        Self {
            target_layout,
            layout_mapper,
            word_list,
        }
    }

    pub fn next_word(&mut self) -> WordInput {
        WordInput::new(self.word_list.next_word())
    }
}

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

pub struct WordIter(<Vec<&'static str> as IntoIterator>::IntoIter);

impl Iterator for WordIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct WordInput {
    expected: String,
    entered: String,
}

impl WordInput {
    pub fn new(expected: impl Into<String>) -> Self {
        Self {
            expected: expected.into(),
            entered: String::new(),
        }
    }

    pub fn push(&mut self, letter: char) {
        self.entered.push(letter);
    }

    pub fn pop(&mut self) {
        self.entered.pop();
    }

    pub fn to_line(&self) -> Line<'static> {
        let mut spans = Vec::new();
        let mut current_ty = LetterType::Valid;
        let mut current_buf = String::new();

        let mut expected = self.expected.chars();
        let mut entered = self.entered.chars();

        loop {
            match (expected.next(), entered.next()) {
                (None, None) => break,
                (None, Some(ent)) => Self::append(
                    &mut spans,
                    &mut current_ty,
                    &mut current_buf,
                    ent,
                    LetterType::Invalid,
                ),
                (Some(exp), None) => Self::append(
                    &mut spans,
                    &mut current_ty,
                    &mut current_buf,
                    exp,
                    LetterType::Placeholder,
                ),
                (Some(exp), Some(ent)) => Self::append(
                    &mut spans,
                    &mut current_ty,
                    &mut current_buf,
                    ent,
                    if exp == ent {
                        LetterType::Valid
                    } else {
                        LetterType::Invalid
                    },
                ),
            }
        }

        if !current_buf.is_empty() {
            spans.push(Span::styled(current_buf, current_ty.style()));
        }

        spans.into()
    }

    fn append(
        spans: &mut Vec<Span<'_>>,
        current_ty: &mut LetterType,
        current_buf: &mut String,
        next_char: char,
        next_ty: LetterType,
    ) {
        if *current_ty != next_ty {
            if !current_buf.is_empty() {
                let buf = replace(current_buf, String::new());
                spans.push(Span::styled(buf, current_ty.style()));
            }
            *current_ty = next_ty;
        }
        current_buf.push(next_char);
    }

    pub fn is_correct(&self) -> bool {
        self.expected == self.entered
    }
}

#[derive(PartialEq, Eq)]
enum LetterType {
    Valid,
    Invalid,
    Placeholder,
}

impl LetterType {
    pub fn style(&self) -> Style {
        match self {
            LetterType::Valid => Style::new(),
            LetterType::Invalid => Style::new().red(),
            LetterType::Placeholder => Style::new().dark_gray(),
        }
    }
}

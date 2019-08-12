use crate::english_text::EnglishText::{Invalid, Likely, Possible, UnLikely};
use std::collections::HashMap;
use std::str;
use std::str::Utf8Error;

#[derive(Debug, PartialEq)]
pub enum EnglishText {
    Likely(String),
    Possible(String),
    UnLikely(String),
    Invalid,
}

struct DecipheredText {
    text: String,
    counts: HashMap<char, i32>,
}

impl DecipheredText {
    pub fn from_str(to_count: &str) -> DecipheredText {
        let mut counts = HashMap::new();

        for c in to_count.chars() {
            match counts.get(&c).cloned() {
                Some(current_count) => counts.insert(c, current_count + 1),
                None => counts.insert(c, 1),
            };
        }
        DecipheredText {
            text: to_count.to_string(),
            counts,
        }
    }

    pub fn from_bytes(to_count: &Vec<u8>) -> Result<DecipheredText, Utf8Error> {
        let converted = str::from_utf8(to_count)?;
        Ok(DecipheredText::from_str(converted))
    }

    fn count_total(&self) -> i32 {
        self.counts.values().fold(0, |acc, i| acc + i)
    }

    fn count_vowel(&self) -> i32 {
        self.counts
            .iter()
            .filter(|p| is_vowel(p.0))
            .map(|p| p.1)
            .fold(0, |acc, i| acc + i)
    }

    fn count_consonant(&self) -> i32 {
        self.counts
            .iter()
            .filter(|p| is_consonant(p.0))
            .map(|p| p.1)
            .fold(0, |acc, i| acc + i)
    }

    fn count_spaces(&self) -> i32 {
        self.counts
            .iter()
            .filter(|p| p.0 == &' ')
            .map(|p| p.1)
            .fold(0, |acc, i| acc + i)
    }

    fn count_alphabetic(&self) -> i32 {
        self.counts
            .iter()
            .filter(|p| char::is_ascii_alphabetic(p.0))
            .map(|p| p.1)
            .fold(0, |acc, i| acc + i)
    }

    fn count_capital(&self) -> i32 {
        self.counts
            .iter()
            .filter(|p| char::is_ascii_uppercase(p.0))
            .map(|p| p.1)
            .fold(0, |acc, i| acc + i)
    }

    fn count_lower(&self) -> i32 {
        self.counts
            .iter()
            .filter(|p| char::is_ascii_lowercase(p.0))
            .map(|p| p.1)
            .fold(0, |acc, i| acc + i)
    }

    fn passes_capital_ratio(&self) -> bool {
        5 * self.count_capital() < self.count_lower()
    }

    fn passes_vowel_ratio(&self) -> bool {
        self.count_vowel() < self.count_consonant()
    }

    fn passes_alphanumeric_ratio(&self) -> bool {
        (self.count_alphabetic() as f32 / self.count_total() as f32) > 0.75
    }

    fn passes_space_ratio(&self) -> bool {
        (self.count_spaces() as f32 / self.count_total() as f32) > 0.05
    }

    fn grade(&self) -> i32 {
        let mut grade = 0;
        if self.passes_alphanumeric_ratio() {
            grade += 1
        }
        if self.passes_capital_ratio() {
            grade += 1
        }
        if self.passes_vowel_ratio() {
            grade += 1
        }
        if self.passes_space_ratio() {
            grade += 1
        }
        grade
    }

    pub fn to_english_text(&self) -> EnglishText {
        match self.grade() {
            4 => Likely(self.text.clone()),
            3 => Possible(self.text.clone()),
            _ => UnLikely(self.text.clone()),
        }
    }
}

pub fn bytes_to_english(bytes: &Vec<u8>) -> EnglishText {
    match DecipheredText::from_bytes(bytes) {
        Ok(text) => text.to_english_text(),
        Err(_) => Invalid,
    }
}

fn is_vowel(c: &char) -> bool {
    match char::to_ascii_lowercase(c) {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn is_consonant(c: &char) -> bool {
    match char::to_ascii_lowercase(c) {
        'b' | 'c' | 'd' | 'f' | 'g' => true,
        'h' | 'j' | 'k' | 'l' | 'm' => true,
        'n' | 'p' | 'q' | 'r' | 's' => true,
        't' | 'v' | 'w' | 'x' | 'y' => true,
        'z' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_english() {
        assert_eq!(
            DecipheredText::from_str("000000").to_english_text(),
            EnglishText::UnLikely("000000".to_string())
        );
        assert_eq!(
            DecipheredText::from_str("QZQZQX").to_english_text(),
            EnglishText::UnLikely("QZQZQX".to_string())
        );
        assert_eq!(
            DecipheredText::from_str("||||||").to_english_text(),
            EnglishText::UnLikely("||||||".to_string())
        );
    }

    #[test]
    fn test_is_english() {
        assert_eq!(
            DecipheredText::from_str("Hello world").to_english_text(),
            EnglishText::Likely("Hello world".to_string())
        );
        assert_eq!(
            DecipheredText::from_str("The quick brown fix jumps over the lazy dog")
                .to_english_text(),
            EnglishText::Likely("The quick brown fix jumps over the lazy dog".to_string())
        );
    }
}

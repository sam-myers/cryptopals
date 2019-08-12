use serialize::base64::{self, ToBase64};
use std::fmt;
use std::u8;

use crate::english_text::*;

pub enum XorSearchResult {
    Found(String, XorKey),
    NotFound,
}

#[derive(Debug, PartialEq)]
pub struct XorKey {
    pub key: Vec<u8>,
}

pub struct XorSearch {
    cypher_text: Vec<u8>,
}

impl XorSearch {
    pub fn new(cypher_text: Vec<u8>) -> XorSearch {
        XorSearch { cypher_text }
    }

    pub fn search_single_bit(&self) -> XorSearchResult {
        for k in 0..u8::MAX {
            let key = XorKey { key: vec![k] };
            let text = xor(&self.cypher_text, &key.key);

            match bytes_to_english(&text) {
                EnglishText::Likely(s) => return XorSearchResult::Found(s, key),
                _ => (),
            }
        }

        XorSearchResult::NotFound
    }
}

impl XorKey {
    fn to_base64(&self) -> String {
        self.key.to_base64(base64::STANDARD)
    }
}

impl fmt::Display for XorKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XorKey<{}>", self.to_base64())
    }
}

pub fn xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..bytes.len() {
        let key_idx = i % key.len();
        result.push(bytes[i] ^ key[key_idx]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_single_byte_key() {
        assert_eq!(
            xor(&[0b00000000, 0b00000001, 0b11111111], &[0b00000111]),
            &[0b00000111, 0b00000110, 0b11111000]
        );
    }

    #[test]
    fn test_xor_multi_byte_key() {
        assert_eq!(
            xor(
                &[0b00000000, 0b00000001, 0b11111111],
                &[0b00000111, 0b11100000]
            ),
            &[0b00000111, 0b11100001, 0b11111000]
        );
    }
}

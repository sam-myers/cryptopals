extern crate rustc_serialize as serialize;

use serialize::hex::FromHex;

mod english_text;
mod set_1;
mod xor;

use crate::english_text::{bytes_to_english, EnglishText};
use std::u8;

fn main() {
    let cipher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        .from_hex()
        .unwrap();

    for k in 0..u8::MAX {
        let key = vec![k];
        let text = xor::xor(&cipher_text, &key);

        match bytes_to_english(&text) {
            EnglishText::Likely(s) => println!("key:{} \"{}\"", k as char, s),
            _ => (),
        }
    }
}
